use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111354830: FileFormat = FileFormat {
    id: 111_354_830,
    source_type: SourceType::Wikidata,
    name: "Yamaha Tyros 2 custom drum voice file",
    extensions: &["tvd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
