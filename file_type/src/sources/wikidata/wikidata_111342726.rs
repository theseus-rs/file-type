use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111342726: FileFormat = FileFormat {
    id: 111_342_726,
    source_type: SourceType::Wikidata,
    name: "SPPack sound sample",
    extensions: &["sppack"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
