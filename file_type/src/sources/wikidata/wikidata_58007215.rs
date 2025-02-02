use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_58007215: FileFormat = FileFormat {
    id: 58_007_215,
    source_type: SourceType::Wikidata,
    name: "Visual Basic File",
    extensions: &["vb"],
    media_types: &["text/x-vba", "text/x-vbnet"],
    internal_signatures: &[],
    related_formats: &[],
};
