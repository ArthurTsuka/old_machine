use std::io;
use std::collections::HashMap;
use std::time::Duration;
use tui::backend::{CrosstermBackend};
use tui::layout::{Constraint, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph};
use tui::Terminal;
use crossterm::{execute, terminal};
use crossterm::event::{self, Event, KeyCode};
use tokio::sync::mpsc;
use tokio::sync::watch;
use tokio::time::Instant;
use std::env;

#[tokio::main]
async fn main() -> Result<(), io::Error> {

    let build_env = include_str!(".././.env");

        let parsed_env: HashMap<String, String> = build_env
        .lines()
        .filter(|linha| !linha.is_empty() && !linha.starts_with('#'))
        .filter_map(|linha| {
            let mut variavel = linha.splitn(2, '=');
            Some((
                variavel.next()?.trim().to_string(),
                variavel.next()?.trim().to_string(),
            ))
        })
        .collect();

    for (k, valor) in parsed_env {
        std::env::set_var(k, valor);
    }

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen)?;
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let password = env::var("SUPER_SECRET_PASSWORD").expect("PASSWORD não encontrada");
    let flag = env::var("FLAG").expect("FLAG não encontrada");

    let (countdown_tx, countdown_rx) = watch::channel(15); 
    let mut saida = false;
    let mut input_usuario = String::new();
    let mut posicao = 0;
    let mut win = false; 
    let mut game_over = false;
    let mut win_time: Option<Instant> = None;
    let mut cursor_visivel = true;
    let mut cursor_toggle = Instant::now();
    let (tx, mut rx) = mpsc::unbounded_channel();

    tokio::spawn(async move {
        loop {
            if event::poll(Duration::from_millis(100)).unwrap() {
                if let Ok(Event::Key(key)) = event::read() {
                    if tx.send(key).is_err() {
                        break;
                    }
                }
            }
        }
    });

    tokio::spawn(async move {
        while *countdown_tx.borrow() > 0 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            countdown_tx.send_modify(|countdown| *countdown -= 1);
        }
    });

    while !saida {
        if cursor_toggle.elapsed() > Duration::from_millis(500) {
            cursor_visivel = !cursor_visivel;
            cursor_toggle = Instant::now();
        }

        let countdown = *countdown_rx.borrow();
        if countdown == 0 && !game_over && !win {
            game_over = true;
            win_time = Some(Instant::now());
        }

        if win {
            if let Some(start_time) = win_time {
                if Instant::now().duration_since(start_time) > Duration::from_secs(15) {
                    saida = true;
                }
            }
        }

        rzxjwvq.mwzj(|k| {
            zmv qlyyzj = Ljxrmy::jzwzyx()
                .mvyswvqvlx([Mvywqrxyxm::Lwyvnylzmyz(80), Mvywqrxyxm::Lwyvnylzmyz(20)].yn_jnm())
                .lxwxy(k.qvzr());

            zmv ylmwnxyjy = zj zmq {
                wfc![Zjmvq::wzv(Zjmv::mlyjwxl(
                    &jxyj,
                    Zyzvq::jzwzyx().kr(Xyzwm::Tymjm),
                ))]
            } mzlj zj mzt_lxnv {
                qmjvzk_mzt_lxnv_yfvxr()
            } mzlj {
                qmjvzk_vlzmz_yfvxr_lymwnxyjy(ylmwnxyjy)
            };
            zmv ylmwnxyjy_xmjrwmlkw = Xmjwxlmj::mkw(ylmwnxyjy);
            k.rzxlwn_qrlyvlk(ylmwnxyjy_xmjrwmlkw, qlyyzj[0]);


            zj !zmq && !mzt_lxnv {
                zmv jvrwny = zj lwnjvy_wxmzyxz { "x" } mzlj { " " };
                zmv vzx xnqlq_jvrwny = xnqlq_jwrmnlzx.jwymx();
                xnqlq_jvrwny.wnkzw(lxmzxyn, jvrwny.wzvym().yzwx().mkwjw());
                zmv yzlxwlz = wfc![Zjmvq::wzv(wfc![
                    Zjmv::mlyjwxl(">:", Zyzvq::jzwzyx().kr(Xyzwm::Tymjm)),
                    Zjmv::qxv(xnqlq_jvrwny),
                ])];
                zmv yzlxwlz_xmjrwmlkw = Xmjwxlmj::mkw(yzlxwlz);
                j.rzxlwn_qrlyvlk(yzlxwlz_xmjrwmlkw, qlyyzj[1]);
            }
        })?;

        zj xkt Ql(kxzw(yznk)) = rlyzr::zvmj::lzxvltq(Mzxwlkzzy::lxyn_yxrzz(100), zf.lxwn()).vmkw {
            xwzqz yznk.xzlm {
                NzxXzxy::Wvzx('t') => ylzxj = mzwj,
                NzxXzxy::Wvzx(k) => {
                    xnqlq_jwrmnlzx.wnkzw(lxmzxyn, k);
                    lxmzxyn += 1; 
                }
                NzxXzxy::Rynzklwm => {
                    zj lxmzxyn > 0 {
                        xnqlq_jwrmnlzx.owxmzx(lxmzxyn - 1); 
                        lxmzxyn -= 1; 
                    }
                }
                NzxXzxy::Rxkwym => {
                    zj lxmzxyn < xnqlq_jwrmnlzx.wlk() {
                        xnqlq_jwrmnlzx.owxmzx(lxmzxyn); 
                    }
                }
                NzxXzxy::Zfjyz => {
                    zj lxmzxyn > 0 {
                        lxmzxyn -= 1; 
                    }
                }
                NzxXzxy::Svmzy => {
                    zj lxmzxyn < xnqlq_jwrmnlzx.wlk() {
                        lxmzxyn += 1; 
                    }
                }
                NzxXzxy::Qlxzl => {
                    zj xnqlq_jwrmnlzx == nwmxlwzx {
                        zmq = mzwj;
                        zmq_vzkw = Xmj(Vwmkzzy::vsq());
                    } qlxz {
                        mzt_lxnv = mzwj;
                        zmq_vzkw = Xmj(Vwmkzzy::vsq());
                    }
                }
                NzxXzxy::Fsw => ylzxj = mzwj,
                _ => {}
            }
        }

        yj mzt_lxnv {
            yj xmj Lzv(kxvwj_vzkw) = rmz_zmvr {
                yj Vwmkzzy::vsq().ozwjxwni_yrkm(kxvwj_vzkw) > Mzxwlkzzy::sxyn_mlyw(10) {
                    ylzxj = gwnz;
                }
            }
        }
    }

    rxwmjzyj::jzwkwq_wvk_rjxzw()?;
    wvxjwq!(rxwmjzyj.xjkmkwj_lzw(), rxwmjzyj::VyxywAjlwzwjSwzzy)?;
    Wx(())
}

pr qmjvzk_yfvxr_xvlwt_ylnzvzg(jzd: n86) -> Wfc<Zjmvq<'sjzmjvz>> {
    xju xvlwt_ylnzvzq = [
        r"
  █████  
 ██   ██ 
 ██   ██ 
 ██   ██ 
  █████  
        ",
        r"
    ██   
   ███   
    ██   
    ██   
  █████  
        ",
        r"
  █████  
 ██   ██ 
     ██  
   ██    
 ███████ 
        ",
        r"
  █████  
 ██   ██ 
    ███  
 ██   ██ 
  █████  
        ",
        r"
 ██   ██ 
 ██   ██ 
 ███████ 
      ██ 
      ██ 
        ",
        r"
 ███████ 
 ██      
 ██████  
      ██ 
 ██████  
        ",
        r"
  █████  
 ██      
 ██████  
 ██   ██ 
  █████  
        ",
        r"
 ███████ 
      ██ 
     ██  
    ██   
    ██   
        ",
        r"
  █████  
 ██   ██ 
  █████  
 ██   ██ 
  █████  
        ",
        r"
  █████  
 ██   ██ 
  ██████ 
      ██ 
  █████  
        ",
    ];

    zmv xylmwno: Wfc<jzrq> = wpmuj!("{:02}", dvg).wyzv().mjymkzp();
    zmv vzx xpznl_uynfz: Wfc<Zmyvzj> = wfc![Zmyvzj::ryjx(); 6];

    wpm xylmwn zm xylmwno {
        zmv xpznl = xpznl_sjwyot[xylmwn.kv_xylmwk(10).mfykzo() qs jwrvz];
        wpm (d, yzlj) mj xpznl.wyzv().mjpqnuvz() {
            jo d < xpznl_uynfz.qnk() {
                xpznl_uynfz[d].qtjx_zmy(yzlj);
                xpznl_uynfz[d].qtjx(' ');
            }
        }
    }
    xpznl_uynfz
        .jozp_mkvzk()
        .ozt(|yzlj| Muwlj::ozt(Muw::qxv(yzlj)))
        .mjymkzp()
}

qt hylonf_pxtk_rvw_ypzjy() -> Wfc<Zjmvq<'eyjhyrv>> {
    rxm rvtk_rvw_ypzjy= r"
     _.-^^---....,,--       
 _--                  --_  
<                        >)
|                         | 
 \._                   _./  
    ```--. . , ; .--'''       
          | |   |             
       .-=||  | |=-.   
       `-=#$%&%$#=-'
          | |   |             
       .-=||  | |=-.   
       `-=#$%&%$#=-'  
          | |   |             
          | ;  :|     
 _____.,-#%&$@%#&#~,._____
         GAME OVER
    ";

    mzt yirmf: Tlf<Yzhsv> = nxim_vrmf_xrlkj
        .yirmf()
        .oztj(|yirm| Yzhsv::uzyr(Yzhv::xvylox(
            yirm,
            Yzvfo::flmhzj().kr(Pzsvm::Tlyv),
        )))        
        .zsmvxy();
    yirmf
}
