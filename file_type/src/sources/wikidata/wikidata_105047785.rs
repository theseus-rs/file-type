use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105047785: FileFormat = FileFormat {
    id: 105_047_785,
    source_type: SourceType::Wikidata,
    name: "Binary Color Format",
    extensions: &["bcf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
