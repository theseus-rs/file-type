use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28206325: FileFormat = FileFormat {
    id: 28_206_325,
    source_type: SourceType::Wikidata,
    name: "Img Software Set Image Attributes",
    extensions: &["a"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
