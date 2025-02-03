use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_116446733: FileFormat = FileFormat {
    id: 116_446_733,
    source_type: SourceType::Wikidata,
    name: "Microsoft Profit 1.0 Company File",
    extensions: &["pft"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
