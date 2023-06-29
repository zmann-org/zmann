import React, { useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Button } from '@carbon/react'

export default function Greet() {
  const handleButtonClick = () => {
    invoke<string>('greet', { name: 'Next.js' })
      .then(console.log)
      .catch(console.error)
  }

  useEffect(() => {
    // Add any necessary initialization code here
  }, [])

  return (
    <>
      <Button onClick={handleButtonClick}>Invoke greet</Button>
    </>
  )
}