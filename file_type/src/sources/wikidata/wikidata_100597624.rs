use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_100597624: FileFormat = FileFormat {
    id: 100_597_624,
    source_type: SourceType::Wikidata,
    name: "Minitab Project, version 15+",
    extensions: &["mpj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
