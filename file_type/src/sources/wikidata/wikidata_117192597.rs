use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117192597: FileFormat = FileFormat {
    id: 117_192_597,
    source_type: SourceType::Wikidata,
    name: "Acrobat TouchUp Image",
    extensions: &["ai", "pdf", "pdp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
