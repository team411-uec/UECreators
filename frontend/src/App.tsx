import { useEffect, useState } from 'react'

function App() {
  const [message, setMessage] = useState<string>('読み込み中...')

  useEffect(() => {
    // バックエンド(Axum)のAPIを叩く
    const apiUrl = import.meta.env.VITE_API_URL || 'http://localhost:3000';
    
    fetch(`${apiUrl}/api/hello`)
      .then((res) => res.json())
      .then((data) => {
        setMessage(data.message)
      })
      .catch((err) => {
        console.error("API通信エラー:", err)
        setMessage('バックエンドに接続できませんでした。Axumサーバーは起動していますか？')
      })
  }, [])

  return (
    <div style={{ padding: '2rem', fontFamily: 'sans-serif' }}>
      <h1>クリエイター交流プラットフォーム PoC</h1>
      <div style={{ padding: '1rem', background: '#f0f0f0', borderRadius: '8px' }}>
        <h3>バックエンドからのメッセージ：</h3>
        <p style={{ color: 'blue', fontWeight: 'bold' }}>{message}</p>
      </div>
    </div>
  )
}

export default App