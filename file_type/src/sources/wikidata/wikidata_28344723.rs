use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28344723: FileFormat = FileFormat {
    id: 28_344_723,
    source_type: SourceType::Wikidata,
    name: "Turbo Pascal chain file",
    extensions: &["chn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
