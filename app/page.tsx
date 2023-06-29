"use client";
import { Button } from '@carbon/react'

import Image from 'next/image'
import styles from './page.module.css'
import Greet from './test';

export default function Home() {
  return (
    <main className={styles.main}>
      <Button>Example usage</Button>
      <div className='test'>hello world</div>
      <Greet/>
    </main>
  )
}
