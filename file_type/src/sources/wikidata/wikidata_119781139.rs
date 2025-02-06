use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_119781139: FileFormat = FileFormat {
    id: 119_781_139,
    source_type: SourceType::Wikidata,
    name: "Street Atlas USA File",
    extensions: &["saf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
