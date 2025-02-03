use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_100597624: FileFormat = FileFormat {
    id: 100_597_624,
    source_type: SourceType::Wikidata,
    name: "Minitab Project, version 15+",
    extensions: &["mpj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
