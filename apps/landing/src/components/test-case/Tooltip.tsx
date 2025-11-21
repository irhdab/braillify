'use client'

import { VStack } from '@devup-ui/react'
import { useRef, useState } from 'react'
import { useEffect } from 'react'

export default function Tooltip({ children }: { children: React.ReactNode }) {
  const [viewportWidth, setViewportWidth] = useState(0)
  const ref = useRef<HTMLDivElement>(null)
  if (typeof window !== 'undefined') setViewportWidth(window.innerWidth)
  useEffect(() => {
    const handleResize = () => {
      setViewportWidth(window.innerWidth)
    }

    window.addEventListener('resize', handleResize)

    return () => {
      window.removeEventListener('resize', handleResize)
    }
  }, [])

  return (
    <VStack
      ref={(el) => {
        if (!el) return

        ref.current = el

        const mo = new ResizeObserver((entries) => {
          entries.forEach((entry) => {
            const target = entry.target as HTMLDivElement
            const x = target.offsetLeft
            const width = target.offsetWidth
            if (x + width > viewportWidth) {
              target.style.right = '16px'
            }
          })
        })

        mo.observe(el)

        return () => mo.disconnect()
      }}
      _groupHover={{
        display: 'flex',
      }}
      bg="rgba(0, 0, 0, 0.75)"
      borderRadius="4px"
      display="none"
      justifyContent="center"
      maxW="calc(100vw - 32px)"
      onMouseEnter={(e) => {
        e.stopPropagation()
      }}
      onMouseLeave={(e) => {
        e.stopPropagation()
      }}
      pos="absolute"
      px="10px"
      py="8px"
      transform="translateY(10px)"
      transition="all 0.3s ease-in-out"
    >
      {children}
    </VStack>
  )
}
