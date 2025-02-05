use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83370520: FileFormat = FileFormat {
    id: 83_370_520,
    source_type: SourceType::Wikidata,
    name: "Midi Sample dump Format",
    extensions: &["sds"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
