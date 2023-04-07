import { ApiPromise, WsProvider } from '@polkadot/api'

const websocket_addr = 'ws://127.0.0.1:9944'

const subscribeSystemEvents = async (api: ApiPromise) => {
  console.log('Subscribing to system events...')
  api.query.system.events((events: any) => {
    console.log('Received system events: ')
    events.forEach((record: any) => {
      const { event, phase } = record
      const types = event.typeDef
      console.log(`method: ${event.method}`)
      console.log(`meta: ${event.meta}`)
      event.data.forEach((data: any, index: any) => {
        console.log(`eventData:${types[index].type}: ${data.toString()}`)
      })
    })
  })
}

async function main() {
  const provider = new WsProvider(websocket_addr)
  const api = await ApiPromise.create({ provider })
  await subscribeSystemEvents(api)

  process.exit(0)
}

main().catch(console.error)
