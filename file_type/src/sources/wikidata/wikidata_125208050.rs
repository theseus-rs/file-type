use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125208050: FileFormat = FileFormat {
    id: 125_208_050,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project XML",
    extensions: &["mspxml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
