use tokio::sync::OnceCell;

async fn html() -> &'static str {
    r#"<!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Document</title>
        <style>
          html {
            font-family: "Verdana";
          }
          body {
            background-color: #e3e8e6;
            display: grid;
            place-items: center;
            height: 100dvh;
            margin: 0;
          }

          .main-table-containter {
            border-radius: 3vmin;
            background-color: #f6f6f4;
            max-width: 900px;
            width: 100%;

            padding: 1rem 1rem;
            box-shadow: 0px 2px 5px 0px #00000033;
          }

          .title-table-container {
            display: flex;
            justify-content: space-between;
            margin-bottom: 1.2rem;
            align-items: center;
            padding: 0rem 1.5rem;
          }

          .subtitle {
            font-weight: 600;
          }

          .select-button {
            padding: 0.45rem 1rem;
            padding-right: 1.5rem;
            position: relative;
            border-radius: 0.5rem;
            border: 2px solid lightgrey;
            font-weight: 600;
            border: none;
            box-shadow: 0px 1px 2px 0px #0000003b, -1px 0px 2px 0px #ffffff33;
          }

          .select-button:after {
            content: " ";
            position: absolute;
            height: 0.7vmin;
            width: 0.7vmin;
            border: 1px solid black;
            border-bottom: none;
            border-left: none;
            right: 10%;
            top: 30%;
            transform: rotate(134deg);
          }

          .dot {
            height: 15px;
            width: 15px;
          }

          table {
            width: 100%;
            font-weight: 600;
            font-size: 11px;
            border-collapse: COLLAPSE;
            color: #474747;
          }

          tr {
            transition: background-color 0.2s;
          }
          tr:hover {
            background-color: #fffffd;
          }
          td {
            padding: 1rem;
            border: none;
          }

          td {
            text-align: center;
          }
          td:first-child {
            border-radius: 1.5rem 0rem 0rem 1.5rem;
            text-align: left;
          }

          td:last-child {
            border-radius: 0rem 1.5rem 1.5rem 0rem;
          }

          .icono-texto {
            display: flex;
            align-items: CENTER;
          }

          .icono-texto > div {
            margin-left: 10px;
          }
          .contenedor-svg {
            background-color: #e090c9;
            border-radius: 0.7rem;
            padding: 10px;
          }
          .dolar {
            height: 15px;
            width: 15px;
          }

          .pendiente {
            background-color: #fde9d3;
            color: #e8aa71;
            padding: 0.5rem;
            border-radius: 0.5rem;
          }

          .completado {
            background-color: #e6ece9;
            color: #80aba4;
            padding: 0.5rem;
            border-radius: 0.5rem;
          }

          .cancelado {
            background-color: #f3dedc;
            color: #cf5858;
            padding: 0.5rem;
            border-radius: 0.5rem;
          }

          .otro-dolar {
            background-color: #b3afec;
          }

          @media (max-width: 750px) {
            td:nth-child(2) {
              display: none;
            }

            td:nth-child(3) {
              display: none;
            }
          }
        </style>
      </head>
      <body>
        <div class="main-table-containter">
          <div class="title-table-container">
            <div class="subtitle">{ title }</div>
          </div>
          <div>
            <table>
              <tbody>
                <tr>
                  <td>
                    <div class="icono-texto">
                      <div class="contenedor-svg">
                        <svg
                          class="dolar"
                          viewBox="0 0 288 512"
                          width="100"
                          title="dollar-sign"
                        >
                          <path
                            d="M209.2 233.4l-108-31.6C88.7 198.2 80 186.5 80 173.5c0-16.3 13.2-29.5 29.5-29.5h66.3c12.2 0 24.2 3.7 34.2 10.5 6.1 4.1 14.3 3.1 19.5-2l34.8-34c7.1-6.9 6.1-18.4-1.8-24.5C238 74.8 207.4 64.1 176 64V16c0-8.8-7.2-16-16-16h-32c-8.8 0-16 7.2-16 16v48h-2.5C45.8 64-5.4 118.7.5 183.6c4.2 46.1 39.4 83.6 83.8 96.6l102.5 30c12.5 3.7 21.2 15.3 21.2 28.3 0 16.3-13.2 29.5-29.5 29.5h-66.3C100 368 88 364.3 78 357.5c-6.1-4.1-14.3-3.1-19.5 2l-34.8 34c-7.1 6.9-6.1 18.4 1.8 24.5 24.5 19.2 55.1 29.9 86.5 30v48c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16v-48.2c46.6-.9 90.3-28.6 105.7-72.7 21.5-61.6-14.6-124.8-72.5-141.7z"
                          />
                        </svg>
                      </div>
                      <div>Salario</div>
                    </div>
                  </td>
                  <td>May 12, 2024</td>
                  <td>El grupo gutsky</td>
                  <td>#F6F6F4</td>
                  <td><div class="pendiente">Pendiente</div></td>
                  <td>$9,000</td>
                  <td>
                    <svg
                      class="dot"
                      viewBox="0 0 512 512"
                      width="100"
                      title="ellipsis-h"
                    >
                      <path
                        d="M328 256c0 39.8-32.2 72-72 72s-72-32.2-72-72 32.2-72 72-72 72 32.2 72 72zm104-72c-39.8 0-72 32.2-72 72s32.2 72 72 72 72-32.2 72-72-32.2-72-72-72zm-352 0c-39.8 0-72 32.2-72 72s32.2 72 72 72 72-32.2 72-72-32.2-72-72-72z"
                      />
                    </svg>
                  </td>
                </tr>

                <tr>
                  <td>
                    <div class="icono-texto">
                      <div class="contenedor-svg otro-dolar">
                        <svg
                          class="dolar"
                          viewBox="0 0 288 512"
                          width="100"
                          title="dollar-sign"
                        >
                          <path
                            d="M209.2 233.4l-108-31.6C88.7 198.2 80 186.5 80 173.5c0-16.3 13.2-29.5 29.5-29.5h66.3c12.2 0 24.2 3.7 34.2 10.5 6.1 4.1 14.3 3.1 19.5-2l34.8-34c7.1-6.9 6.1-18.4-1.8-24.5C238 74.8 207.4 64.1 176 64V16c0-8.8-7.2-16-16-16h-32c-8.8 0-16 7.2-16 16v48h-2.5C45.8 64-5.4 118.7.5 183.6c4.2 46.1 39.4 83.6 83.8 96.6l102.5 30c12.5 3.7 21.2 15.3 21.2 28.3 0 16.3-13.2 29.5-29.5 29.5h-66.3C100 368 88 364.3 78 357.5c-6.1-4.1-14.3-3.1-19.5 2l-34.8 34c-7.1 6.9-6.1 18.4 1.8 24.5 24.5 19.2 55.1 29.9 86.5 30v48c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16v-48.2c46.6-.9 90.3-28.6 105.7-72.7 21.5-61.6-14.6-124.8-72.5-141.7z"
                          />
                        </svg>
                      </div>
                      <div>Salario</div>
                    </div>
                  </td>
                  <td>May 12, 2024</td>
                  <td>El grupo gutsky</td>
                  <td>#F6F6F4</td>
                  <td><div class="cancelado">Cancelado</div></td>
                  <td>$9,000</td>
                  <td>
                    <svg
                      class="dot"
                      viewBox="0 0 512 512"
                      width="100"
                      title="ellipsis-h"
                    >
                      <path
                        d="M328 256c0 39.8-32.2 72-72 72s-72-32.2-72-72 32.2-72 72-72 72 32.2 72 72zm104-72c-39.8 0-72 32.2-72 72s32.2 72 72 72 72-32.2 72-72-32.2-72-72-72zm-352 0c-39.8 0-72 32.2-72 72s32.2 72 72 72 72-32.2 72-72-32.2-72-72-72z"
                      />
                    </svg>
                  </td>
                </tr>
                <tr>
                  <td>
                    <div class="icono-texto">
                      <div class="contenedor-svg">
                        <svg
                          class="dolar"
                          viewBox="0 0 288 512"
                          width="100"
                          title="dollar-sign"
                        >
                          <path
                            d="M209.2 233.4l-108-31.6C88.7 198.2 80 186.5 80 173.5c0-16.3 13.2-29.5 29.5-29.5h66.3c12.2 0 24.2 3.7 34.2 10.5 6.1 4.1 14.3 3.1 19.5-2l34.8-34c7.1-6.9 6.1-18.4-1.8-24.5C238 74.8 207.4 64.1 176 64V16c0-8.8-7.2-16-16-16h-32c-8.8 0-16 7.2-16 16v48h-2.5C45.8 64-5.4 118.7.5 183.6c4.2 46.1 39.4 83.6 83.8 96.6l102.5 30c12.5 3.7 21.2 15.3 21.2 28.3 0 16.3-13.2 29.5-29.5 29.5h-66.3C100 368 88 364.3 78 357.5c-6.1-4.1-14.3-3.1-19.5 2l-34.8 34c-7.1 6.9-6.1 18.4 1.8 24.5 24.5 19.2 55.1 29.9 86.5 30v48c0 8.8 7.2 16 16 16h32c8.8 0 16-7.2 16-16v-48.2c46.6-.9 90.3-28.6 105.7-72.7 21.5-61.6-14.6-124.8-72.5-141.7z"
                          />
                        </svg>
                      </div>
                      <div>Salario</div>
                    </div>
                  </td>
                  <td>May 12, 2024</td>
                  <td>El grupo gutsky</td>
                  <td>#F6F6F4</td>
                  <td><div class="completado">Completado</div></td>
                  <td>$9,000</td>
                  <td>
                    <svg
                      class="dot"
                      viewBox="0 0 512 512"
                      width="100"
                      title="ellipsis-h"
                    >
                      <path
                        d="M328 256c0 39.8-32.2 72-72 72s-72-32.2-72-72 32.2-72 72-72 72 32.2 72 72zm104-72c-39.8 0-72 32.2-72 72s32.2 72 72 72 72-32.2 72-72-32.2-72-72-72zm-352 0c-39.8 0-72 32.2-72 72s32.2 72 72 72 72-32.2 72-72-32.2-72-72-72z"
                      />
                    </svg>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
      </body>
    </html>"#
}

static HTML: OnceCell<&'static str> = OnceCell::const_new();

pub(super) async fn get_html() -> &'static str {
    HTML.get_or_init(html).await
}
