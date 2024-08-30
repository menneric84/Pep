package main

import (
	"log"
	"github.com/gdamore/tcell/v2"
)

type Screen struct {
	s tcell.Screen
	x int
	y int
	mode Mode
}


func (s *Screen) move(movement BasicMovements, distance int) {
	switch movement {
		case Up:
			s.y -= distance
		case Down:
			s.y += distance
		case Left:
			s.x -= distance
		case Right:
			s.x += distance
		default:
			return
	}

	s.s.ShowCursor(s.x, s.y)
}

func (s *Screen) draw(r rune) {
	s.s.SetContent(s.x, s.y, r, nil, tcell.StyleDefault)
	s.x++
	s.s.ShowCursor(s.x, s.y)
}

func (s *Screen) backspace() {
	s.x--
	s.s.SetContent(s.x, s.y, ' ', nil, tcell.StyleDefault)
	s.s.ShowCursor(s.x, s.y)
}

type BasicMovements int

const (
	Up BasicMovements = iota
	Down
	Left
	Right
	DoNothing
)

type Mode int

const (
	Normal Mode = iota
	Insert
)

func isMotion(ev rune) bool {
	switch ev {
		case 'j', 'k', 'h', 'l':
			return true
		default:
			return false
	}
}

func handleMotion(ev rune) BasicMovements {
	switch ev {
		case 'j':
			return Down
		case 'k':
			return Up
		case 'h':
			return Left
		case 'l':
			return Right
		default:
			return DoNothing
	}
}


func main() {
	defStyle := tcell.StyleDefault.Background(tcell.ColorReset).Foreground(tcell.ColorReset)


	// Initialize screen
	s, err := tcell.NewScreen()
	if err != nil {
		log.Fatalf("%+v", err)
	}
	if err := s.Init(); err != nil {
		log.Fatalf("%+v", err)
	}

	screen := Screen{s: s, x: 0, y: 0, mode: Normal}
	screen.s.SetStyle(defStyle)
	screen.s.EnablePaste()
	screen.s.Clear()
	screen.s.SetCursorStyle(tcell.CursorStyle(tcell.CursorStyleBlinkingBlock))
	screen.s.ShowCursor(screen.x, screen.y)


	quit := func() {
		maybePanic := recover()
		s.Fini()
		if maybePanic != nil {
			panic(maybePanic)
		}
	}
	defer quit()

	for {
		// Update screen
		s.Show()

		// Poll event
		ev := s.PollEvent()

		// Process event
		switch ev := ev.(type) {
		case *tcell.EventResize:
			s.Sync()
		case *tcell.EventKey:
			if ev.Key() == tcell.KeyCtrlC {
				return;
				
			} else if ev.Key() == tcell.KeyEscape {
				screen.mode = Normal
				screen.s.SetCursorStyle(tcell.CursorStyle(tcell.CursorStyleBlinkingBlock))
			} else if ev.Key() == tcell.KeyBackspace {
				screen.backspace()
			} else if ev.Rune() != 0 {
				if ev.Rune() == 'i' && screen.mode == Normal {
					screen.mode = Insert
					screen.s.SetCursorStyle(tcell.CursorStyle(tcell.CursorStyleDefault))
				} else if (screen.mode == Insert) {
					screen.draw(ev.Rune())
				} else if isMotion(ev.Rune()) {
					screen.move(handleMotion(ev.Rune()), 1)
				}
			}

		}
	}
}