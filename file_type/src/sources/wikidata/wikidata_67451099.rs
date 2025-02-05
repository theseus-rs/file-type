use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67451099: FileFormat = FileFormat {
    id: 67_451_099,
    source_type: SourceType::Wikidata,
    name: "IBM Softcopy Reader (Bookmanager) Bookshelf (and Book) index file",
    extensions: &["bki"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
