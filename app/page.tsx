"use client";
import { Button, Header, HeaderName, Tag } from '@carbon/react'

import Image from 'next/image'
import styles from './page.module.css'
import Greet from './test';

export default function Home() {
  return (
    <main className={styles.main}>
      <Header aria-label="IBM Platform Name">
        <HeaderName href="#" prefix="Distraze">
          <Tag>Beta</Tag>
        </HeaderName>
      </Header>
      <Button>Example usage</Button>
      <div className='test'>hello world</div>
      <Greet />
    </main>
  )
}
