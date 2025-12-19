import http from 'k6/http';
import { sleep } from 'k6';

export let options = {
    vus: 50, // 50 usuarios virtuales al mismo tiempo
    duration: '10s', // durante 10 segundos
};

export default function () {
    http.get('http://localhost:3000'); // local
    sleep(1);
}