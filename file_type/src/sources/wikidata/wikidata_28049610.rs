use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28049610: FileFormat = FileFormat {
    id: 28_049_610,
    source_type: SourceType::Wikidata,
    name: "Tiny Stuff, high resolution",
    extensions: &["tn3", "tny"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
