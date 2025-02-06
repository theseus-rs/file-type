use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34745761: FileFormat = FileFormat {
    id: 34_745_761,
    source_type: SourceType::Wikidata,
    name: "StarCraft group file",
    extensions: &["grp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
