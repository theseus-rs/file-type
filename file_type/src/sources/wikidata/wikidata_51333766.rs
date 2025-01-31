use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_51333766: FileFormat = FileFormat {
    id: 51_333_766,
    puid: "wikidata/51333766",
    name: "Microsoft Powerpoint Add-In",
    extensions: &["ppa", "ppam"],
    media_types: &["application/mspowerpoint", "application/mspowerpoint"],
    internal_signatures: &[],
    related_formats: &[],
};
