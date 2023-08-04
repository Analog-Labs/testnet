import { ApiPromise, WsProvider, Keyring } from '@polkadot/api';
import { Channel } from 'async-channel';
import { dirname } from 'path';
import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const node_address = 'ws://127.0.0.1:9943';

const stringToHex = (str) => {
    return '0x' + str.split('').map((char) => char.charCodeAt(0).toString(16)).join('');
};

const setup_substrate = async () => {
    const wsProvider = new WsProvider(node_address);
    // const custom_types = await get_custom_types();
    const api = await ApiPromise.create({
        provider: wsProvider
    });
    //         , "types": custom_types
    return api;
};

const pallet_task_add = async (_keyspair, who) => {
    const api = await setup_substrate();
    const keyring = new Keyring({ type: 'sr25519' });
    const keyspair = keyring.addFromUri('//Alice', { name: 'Alice default' });

    const chan = new Channel(0 /* default */);
    const input_task = {
        network: 1,
        cycle: 1,
        frequency: 0,
        hash: 'QmWVZN1S6Yhygt35gQej6e3VbEEffbrVuqZZCQc772uRt7',
        status: 0,
        function: {
            EVMViewWithoutAbi: {
                address: stringToHex('0x3de7086ce750513ef79d14eacbd1282c4e4b0cea'),
                function_signature: "function get_votes_stats() external view returns (uint, uint)",
                input: 2,
            }
        },
    }
    await api.isReady;
    console.log("api.tx.task_meta ---> ", api.tx.taskSchedule);
    const unsub = await api.tx.taskSchedule.insertSchedule(input_task).signAndSend(keyspair, ({ status, events, dispatchError }) => {
        console.log(`Current status is ${status}`);
    });

    await chan.get().then(value => console.log(value), error => console.error(error));
    // chan.close();
};

pallet_task_add();
