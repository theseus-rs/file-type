use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111341591: FileFormat = FileFormat {
    id: 111_341_591,
    source_type: SourceType::Wikidata,
    name: "EMU SoundFont v1.0 bank",
    extensions: &["sbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
