package main

import (
	"log"
	"github.com/gdamore/tcell/v2"
)

// Go does not have enums, so we use a type alias for a more readable code
type Mode int
const (
    Command Mode = iota 
    Write       
)

func (m Mode) String() string {
    switch m {
    case Command:
        return "Command"
    case Write:
        return "Write"
    }
    return "Unknown"
}

type Editor struct  {
    mode Mode
    cursorX int
    cursorY int
}

func initScreen() tcell.Screen {
	defStyle := tcell.StyleDefault.Background(tcell.ColorReset).Foreground(tcell.ColorReset)

	// Initialize screen
	s, err := tcell.NewScreen()

	if err != nil {
		log.Fatalf("%+v", err)
	}

	if err := s.Init(); err != nil {
		log.Fatalf("%+v", err)
	}

	s.SetStyle(defStyle)
	s.EnableMouse()
	s.Clear()

	// Draw initial boxes

	quit := func() {
		maybePanic := recover()
		s.Fini()
		if maybePanic != nil {
			panic(maybePanic)
		}
	}

	defer quit()
    return s
}

func main() {
    screen := initScreen()
    editorState := initState()

	for {
		// Update screen
		screen.Show()

		// Poll event
		ev := screen.PollEvent()

		// Process event
		switch ev := ev.(type) {
            case *tcell.EventResize:
                // re-render the whole screen. Only do on clear or resize
                screen.Sync()
            case *tcell.EventKey:
                if ev.Key() == KeyRune{

		}
	}
}
