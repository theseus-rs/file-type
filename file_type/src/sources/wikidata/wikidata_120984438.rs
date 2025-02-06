use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120984438: FileFormat = FileFormat {
    id: 120_984_438,
    source_type: SourceType::Wikidata,
    name: "Microsoft Publisher Template",
    extensions: &["pub"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
