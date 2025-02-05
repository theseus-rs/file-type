use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58960003: FileFormat = FileFormat {
    id: 58_960_003,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Chart, version 3",
    extensions: &["slc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
