use iced::{
    button, executor, text_input, Align, Button, Column, Command, Length, Row, Text, TextInput,
};

#[derive(Default)]
pub struct Application {
    calc_type: u8,
    nota_atividade: f32,
    nota_prova: f32,
    result: f32,

    button_calc_media: button::State,
    button_calc_prever_prova: button::State,
    input_nota_atividades: text_input::State,
    input_nota_prova: text_input::State,
}

#[derive(Debug, Clone)]
pub enum Message {
    SwipeType(u8),
    NotaAtividadesChange(String),
    NotaProvaChange(String),
}

impl iced::Application for Application {
    type Executor = executor::Default;

    type Message = Message;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let command = Command::none();

        (Self::default(), command)
    }

    fn title(&self) -> String {
        String::from("Notas Univesp - 2022")
    }

    fn update(
        &mut self,
        message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        match message {
            Message::SwipeType(type_calc) => self.calc_type = type_calc,
            Message::NotaAtividadesChange(nota_raw) => {
                self.nota_atividade = parse_text_number(nota_raw);
            }

            Message::NotaProvaChange(nota_raw) => {
                self.nota_prova = parse_text_number(nota_raw);
            }
        }

        self.calc();

        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        let button_media = Button::new(&mut self.button_calc_media, Text::new("Calcular Média"))
            .on_press(Message::SwipeType(0));

        let button_prever_prova = Button::new(
            &mut self.button_calc_prever_prova,
            Text::new("Calcular Nota Prova Minima"),
        )
        .on_press(Message::SwipeType(1));

        let row_buttons = Row::new()
            .push(button_media)
            .push(button_prever_prova)
            .spacing(10);

        let input_nota_atividades = TextInput::new(
            &mut self.input_nota_atividades,
            "Media notas atividades",
            &format_number_input(self.nota_atividade),
            Message::NotaAtividadesChange,
        )
        .width(Length::Units(300))
        .padding(10);

        let input_prova_name = if self.calc_type == 0 {
            "Nota da prova"
        } else {
            "Nota prova minima"
        };

        let input_prova = TextInput::new(
            &mut self.input_nota_prova,
            input_prova_name,
            &format_number_input(self.nota_prova),
            Message::NotaProvaChange,
        )
        .width(Length::Units(300))
        .padding(10);

        let result_text = Text::new(self.result.to_string());

        let center_screen = Column::new()
            .push(row_buttons)
            .push(input_nota_atividades)
            .push(input_prova)
            .push(result_text)
            .align_items(Align::Center)
            .width(Length::Fill)
            .spacing(20);

        Row::new()
            .push(center_screen)
            .height(Length::Fill)
            .align_items(Align::Center)
            .into()
    }
}

impl Application {
    fn calc(&mut self) {
        if self.calc_type == 0 {
            self.result = (self.nota_prova * 0.6) + (self.nota_atividade * 0.4);
        } else {
            self.result = (10. * (self.nota_prova - (self.nota_atividade * 0.4))) / 6.
        }
    }
}

fn format_number_input(input: f32) -> String {
    if input == 0. {
        "".to_owned()
    } else {
        input.to_string()
    }
}

fn parse_text_number(input: String) -> f32 {
    let parsed_nota = input.parse::<f32>();

    if let Ok(nota) = parsed_nota {
        nota
    } else {
        0.0
    }
}
