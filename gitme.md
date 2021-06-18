“Mi è arrivata la Pull Request, e ora che faccio?”
“Beh prima la provi in locale e poi se ti va bene la mergi.”
“E come la provo in locale?”
“Applichi la sua modifica al branch main”
“Ma io ho delle modifiche in locale.”
“Allora prima di tutto crei un branch ‘modifiche', cose che avresti già dovuto fare...”
“Ok ok... Fatto.”
“Ora committa tutto quello che hai fatto nel nuovo branch.”
“Ok, ora?”
“Ora cambi al branch main, e fai una pull del branch main di origin (il repo su GitHub) per sincronizzarti.”
“Così ora il mio branch main è identico a quello del repository su GithHub?”
“Esatto. Ora vai sulla pull request e vedi da dove viene. Vedi dove si trova la sua modifica? Il repo e il branch?” 
“Vedo http://github.com/<utente>/<fork> <branch>”
“Adesso aggiungi un nuovo remoto, chiamalo contributor, con http://github.com/<utente>/<fork>”
“Fatto, ora?”
“Ora fai una pull, e lui ti chiederà da quale remoto e da quale branch”
“Infatti c’è il contributor e poi vedo il suo branch”
“Se ora fai una pull avrai applicato alla tua copia locale del main la stessa modifica della pull request. Fallo e provala”
“Fatto, provata, mi sembra ok”
“Allora puoi approvare la pull request”
“Bene, ma così restano su GitHub. Io voglio le sue modifiche nel mio branch in locale!”
“Torna al tuo branch ‘modifiche’ e fai un rebase del main’
“Cosa è un rebase?”
“Serve ad allinearti al main di origin. In pratica ri-applica le tue modifiche che hai fatto ad una vecchia versione, a partire dalla nuova versione”.
“Non ho capito!”
“Non sei la sola. Pensala così: hai preso una vecchia versione e hai fatto delle modifiche. Ora però ci ha lavorato qualcun altro, e la versione è cambiata. Cosa fai?”
“Mi sparo?”
“Si estinguerebbero presto i programmatori allora. No, la soluzione è di estrarre tutte le modifiche che hai fatto a partire dalla vecchia versione ed applicarle alla nuova versione.”
“E come si fa?”
“Lo fa git, estrae ogni modifica a partire dal vecchio e la ri-applica al nuovo.”
“E funziona?”
“Se le fai piccole e pulite SI, se il tuo codice è una bolgia no. Se cambi spaziature, indentazioni, sposti le cose senza motivo e via discorrendo diventa dura. Cioè si puó fare ma diventa faticosissimo, perché compaiono un sacco di conflitti, e devi risolverli tutti a mano.”
“Ah ecco perché sei così isterico su ‘formatta il codice e rimuovi gli spazi’ prima di committare…”
“Prova.”
“Oh che bello, ha funzionato. Ha fatto tutto lui. Ci sono le mie modifiche e le sue insieme…”
“Vedo che cominci a capire…”
