use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_110015870: FileFormat = FileFormat {
    id: 110_015_870,
    source_type: SourceType::Wikidata,
    name: "InstallShield Executable",
    extensions: &["ex_"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
