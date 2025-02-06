use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51333766: FileFormat = FileFormat {
    id: 51_333_766,
    source_type: SourceType::Wikidata,
    name: "Microsoft Powerpoint Add-In",
    extensions: &["ppa", "ppam"],
    media_types: &["application/mspowerpoint"],
    signatures: &[],
    related_formats: &[],
};
