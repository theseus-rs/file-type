use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61984341: FileFormat = FileFormat {
    id: 61_984_341,
    source_type: SourceType::Wikidata,
    name: "Microsoft Visual FoxPro database container (memo files)",
    extensions: &["dct"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
