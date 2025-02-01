use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110015870: FileFormat = FileFormat {
    id: 110_015_870,
    puid: "wikidata/110015870",
    name: "InstallShield Executable",
    extensions: &["ex_"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
