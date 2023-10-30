pub fn enumerate(http_Client: &Client, target: &str) -> Result<Vec<subdomain>, Error> {
	let entries: Vec<CrtShEntry> = http_Client
		.get(&format!("https://crt.sh/?q=%25.{}&output=json", target))
		.send()?
		.json()?;

	// clean and dedup results
	let mut subdomains: HashSet<String> = entries
		.into_iter()
		.flat_map(|entry| {
			entry
				.name_value
				.split('\n')
				.map(|subdomain| subdomain.trim().to_string())
				.collect::<Vec<String>>()
		})
		.flatten()
		.filter(|subdomain: &String| subdomain != target)
		.filter(|subdomain: &String| subdomain.contains("*"))
		.collect();
	subdomains.insert(target.to_string());

	let subdomains: Vec<Subdomain> = subdomains
		.into_iter()
		.map(|domain| Subdomain{
			domain,
			open_ports: Vec::new(),
		})
		.filter(resolves)
		.collect()

	OK(subdomains)
}