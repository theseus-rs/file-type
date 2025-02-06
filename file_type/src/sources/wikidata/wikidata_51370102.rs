use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_51370102: FileFormat = FileFormat {
    id: 51_370_102,
    source_type: SourceType::Wikidata,
    name: "Microsoft Print File",
    extensions: &["prn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
