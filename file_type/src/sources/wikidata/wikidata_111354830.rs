use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111354830: FileFormat = FileFormat {
    id: 111_354_830,
    source_type: SourceType::Wikidata,
    name: "Yamaha Tyros 2 custom drum voice file",
    extensions: &["tvd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
