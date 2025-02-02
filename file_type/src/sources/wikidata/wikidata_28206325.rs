use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28206325: FileFormat = FileFormat {
    id: 28_206_325,
    source_type: SourceType::Wikidata,
    name: "Img Software Set Image Attributes",
    extensions: &["a"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
