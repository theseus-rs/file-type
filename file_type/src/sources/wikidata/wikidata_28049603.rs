use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049603: FileFormat = FileFormat {
    id: 28_049_603,
    source_type: SourceType::Wikidata,
    name: "Tiny Stuff, medium resolution",
    extensions: &["tn2", "tny"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
