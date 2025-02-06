use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_66663925: FileFormat = FileFormat {
    id: 66_663_925,
    source_type: SourceType::Wikidata,
    name: "OS/2 Metafile",
    extensions: &["met"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
