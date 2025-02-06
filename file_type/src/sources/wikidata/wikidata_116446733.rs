use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116446733: FileFormat = FileFormat {
    id: 116_446_733,
    source_type: SourceType::Wikidata,
    name: "Microsoft Profit 1.0 Company File",
    extensions: &["pft"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
