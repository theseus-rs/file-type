use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_3596392: FileFormat = FileFormat {
    id: 3_596_392,
    source_type: SourceType::Wikidata,
    name: "Microsoft Word document template",
    extensions: &["dot"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
