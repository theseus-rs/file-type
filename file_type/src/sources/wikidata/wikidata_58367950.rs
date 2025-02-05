use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58367950: FileFormat = FileFormat {
    id: 58_367_950,
    source_type: SourceType::Wikidata,
    name: "Microsoft Project file format version 2007",
    extensions: &["mpp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
