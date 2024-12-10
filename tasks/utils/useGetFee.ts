import { EndpointId } from '@layerzerolabs/lz-definitions'
import { deriveConnection } from '../solana'
import getPrioritizationFees from './getFee'

async function main() {
    const { connection } = await deriveConnection(EndpointId.SOLANA_V2_TESTNET)
    const res = await getPrioritizationFees(connection)
    console.log(res)
}

main()
