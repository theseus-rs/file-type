use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_67451099: FileFormat = FileFormat {
    id: 67_451_099,
    source_type: SourceType::Wikidata,
    name: "IBM Softcopy Reader (Bookmanager) Bookshelf (and Book) index file",
    extensions: &["bki"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
