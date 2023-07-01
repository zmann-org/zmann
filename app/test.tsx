import React, { useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Button } from '@carbon/react'

export default function Greet() {
  const handleButtonClick = () => {
    invoke<string>('test')
      .then(console.log)
      .catch(console.error)
  }

  return (
    <>
      <Button onClick={handleButtonClick}>Invoke greet</Button>
    </>
  )
}