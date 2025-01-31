use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111341591: FileFormat = FileFormat {
    id: 111_341_591,
    puid: "wikidata/111341591",
    name: "EMU SoundFont v1.0 bank",
    extensions: &["sbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
