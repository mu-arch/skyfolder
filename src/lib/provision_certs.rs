/*

pub async fn ssl_cert_lifecycle_manager() {

    /*
        let _ = get_dns().await;
        return
     */


    //init the cache
    crate::lib::once::set_tls_cert_cache().await;

    //try downloading a suitable cert from the database to launch the application
    println!("! Blocking while waiting to download or provision TLS cert.");
    let success = download_and_set_cert_cache().await;
    //this panic only happens at start time, during future runtime we don't panic, just silently fail
    if !success {
        panic!("Cannot proceed: failed to get ssl cert to start application. Maybe try again.")
    }


    tokio::spawn(async move {

        //since we ran at the start of the program we can sleep now for a day
        sleep(Duration::from_secs(86400)).await;

        loop {
            //try to refresh our cert from db or generate a new one. if succeed wait another day, if fail retry in 20 mins
            match download_and_set_cert_cache().await {
                true => sleep(Duration::from_secs(86400)).await,
                false => sleep(Duration::from_secs(1200)).await
            }
        }
    });

}

async fn read_and_set_cert_cache(hostname: &str) -> bool {

    let v = match atomic_read_or_write_cert(hostname).await {
        Ok(v) => v,
        Err(e) => {dbg!(e); return false}
    };

    //write v to oncecell
    let mut write_guard = TLS_CERT_CACHE.get().unwrap().write().await;

    *write_guard = v;

    true
}

async fn atomic_read_or_write_cert(hostname: &str) -> Result<CertRow, AppErrorInternal> {

    //attempt to read cert value

    // if not exists provision a new one


    Ok(cert)
}




async fn request_certificate(hostname: &str) -> Result<CertRow, AppErrorInternal> {
    let url = DirectoryUrl::LetsEncrypt;

// Create a directory entrypoint.
    let dir = Directory::from_url(url)?;

// Your contact addresses, note the `mailto:`
    let contact = vec![];

// Generate a private key and register an account with your ACME provider.
    let acc = dir.register_account(contact.clone())?;

// Order a new TLS certificate for a domain.
    let mut ord_new = acc.new_order(hostname, &[])?;

    let auths = ord_new.authorizations()?;


    let ord_csr = loop {
        if let Some(ord_csr) = ord_new.confirm_validations() {
            break ord_csr;
        }

        let auths = ord_new.authorizations()?;

        let challenge = auths.get(0)
            .ok_or_else(|| Custom("Failed to access auths index".to_owned()))?
            .dns_challenge()
            .ok_or_else(|| Custom("Failed to unwrap dns challenge".to_owned()))?;

        set_dns_challenge(&challenge.dns_proof()?).await?;
        challenge.validate(Duration::from_millis(10000))?;

        ord_new.refresh()?;
    };


    let pkey_pri = create_p384_key()?;

// Submit the CSR. This causes the ACME provider to enter a
// state of "processing" that must be polled until the
// certificate is either issued or rejected. Again we poll
// for the status change.
    let ord_cert =
        ord_csr.finalize_pkey(pkey_pri, Duration::from_secs(10))?;

// Finally download the certificate.
    let cert = ord_cert.download_cert()?;


    let cert = CertRow {
        full_chain: cert.certificate().to_owned(),
        private_key: cert.private_key().to_owned(),
    };


    Ok(cert)
}

async fn set_dns_challenge(challenge: &str) -> Result<(), AppErrorInternal> {



    Ok(())

}

 */