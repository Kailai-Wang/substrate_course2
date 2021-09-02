import React, { useEffect, useState } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrate } from './substrate-lib'
import { TxButton } from './substrate-lib/components'

import KittyCards from './KittyCards'

export default function Kitties (props) {
  const { api, keyring } = useSubstrate()
  const { accountPair } = props

  const [kitties, setKitties] = useState([])
  const [status, setStatus] = useState('')

  const fetchKitties = () => {
    let unsubscribe
    // query the kitty count
    api.query.kittiesModule.kittiesCount(count => {
      const c = count.isNone ? 0 : count.unwrap().toNumber()

      let countChanged = true
      let kittyArray = []

      for (let i = 0; i < c; i++) {
        // query (listen) the 'kitties' and 'owner' in a multi query
        api.queryMulti([
          [api.query.kittiesModule.kitties, i],
          [api.query.kittiesModule.owner, i]
        ], ([kitty, owner]) => {
          if (countChanged) {
            // we get notified because kittyCount changed
            // in this case concatenate the kitty arrays and
            // update the state in the end
            kittyArray = [
              ...kittyArray,
              {
                id: i,
                dna: kitty.unwrapOr(null).toU8a(),
                owner: owner.toHuman()
              }
            ]
            if (i === c - 1) {
              setKitties(kittyArray)
              countChanged = false
            }
          } else {
            // we get notified because either kitties or owner changes
            // in this case only update the kitty with the matching index
            // while leaving others untouched
            kittyArray = kittyArray.map(k =>
              k.id !== i
                ? k
                : {
                    id: i,
                    dna: kitty.unwrapOr(null).toU8a(),
                    owner: owner.toHuman()
                  })
            setKitties(kittyArray)
          }
        })
      }
    })
      .then(unsub => {
        unsubscribe = unsub
      })
      .catch(console.error)
  }

  useEffect(fetchKitties, [api, keyring])

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} accountPair={accountPair} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          accountPair={accountPair} label='创建小毛孩' type='SIGNED-TX' setStatus={setStatus}
          attrs={{
            palletRpc: 'kittiesModule',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>
}
