import pytest
from solana.rpc.async_api import AsyncClient
from solana.keypair import Keypair
from solana.transaction import Transaction
from solders.pubkey import Pubkey

# Здесь мы подключаемся к локальной Solana
@pytest.mark.asyncio
async def test_connection():
    client = AsyncClient("http://127.0.0.1:8899")
    resp = await client.get_health()
    assert resp.value == "ok"

# Пример теста: проверка генерации ключа
def test_key_generation():
    kp = Keypair()
    assert isinstance(kp.public_key, Pubkey)
