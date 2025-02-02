use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_34745761: FileFormat = FileFormat {
    id: 34_745_761,
    source_type: SourceType::Wikidata,
    name: "StarCraft group file",
    extensions: &["grp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
