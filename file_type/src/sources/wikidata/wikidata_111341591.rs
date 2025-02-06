use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111341591: FileFormat = FileFormat {
    id: 111_341_591,
    source_type: SourceType::Wikidata,
    name: "EMU SoundFont v1.0 bank",
    extensions: &["sbk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
