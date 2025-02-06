use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_60342641: FileFormat = FileFormat {
    id: 60_342_641,
    source_type: SourceType::Wikidata,
    name: "Microsoft Excel Macro-Enabled Template",
    extensions: &["xltm"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
