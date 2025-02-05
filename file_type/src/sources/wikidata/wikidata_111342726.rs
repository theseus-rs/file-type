use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111342726: FileFormat = FileFormat {
    id: 111_342_726,
    source_type: SourceType::Wikidata,
    name: "SPPack sound sample",
    extensions: &["sppack"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
